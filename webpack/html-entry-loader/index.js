const path = require('path');

const loaderUtils = require('loader-utils');
const { parse } = require('node-html-parser');

const { NormalModule } = require('webpack');

const pluginName = 'HtmlEntryLoader';

function isEntryModule(compilation, module) {
  return compilation.moduleGraph.getIssuer(module) === null;
}

function generateScript(jsFiles, hash) {
  return jsFiles.map((x) => `<script type='text/javascript' src='${x}?${hash}'></script>`).join('');
}

function generateLink(cssFiles, hash) {
  return cssFiles.map((x) => `<link rel='stylesheet' href='${x}?${hash}' />`).join('');
}

function chunkContainsUserRequest(chunk, userRequest, compilation) {
  for (const entryModule of compilation.chunkGraph.getChunkEntryModulesIterable(chunk)) {
    if (entryModule && entryModule.userRequest) {
      return entryModule.userRequest === userRequest;
    }

    if (entryModule && entryModule.dependencies) {
      for (const dependency of entryModule.dependencies) {
        if (dependency.module.userRequest === userRequest) {
          return true;
        }
      }
    }
  }
  return false;
}

function findEntrypointContainingUserRequest(userRequest, compilation) {
  for (const entrypoint of compilation.entrypoints.values()) {
    for (const chunk of entrypoint.chunks) {
      if (chunkContainsUserRequest(chunk, userRequest, compilation)) {
        return entrypoint;
      }
    }
  }

  throw Error();
}

function injectChunks(content, userRequest, compilation) {
  const entrypoint = findEntrypointContainingUserRequest(userRequest, compilation);
  const jsFiles = entrypoint.getFiles().filter((x) => x.endsWith('.js'));
  const cssFiles = entrypoint.getFiles().filter((x) => x.endsWith('.css'));

  const inject = generateScript(jsFiles, compilation.hash) + generateLink(cssFiles, compilation.hash);

  return content.replace('</head>', `${inject}</head>`);
}

class EntryExtractPlugin {
  /* eslint-disable-next-line class-methods-use-this */
  apply(compiler) {
    const entries = {};

    compiler.hooks.thisCompilation.tap(pluginName, (compilation) => {
      compilation.hooks.processAssets.tap(pluginName, () => {
        for (const [userRequest, content] of Object.entries(entries)) {
          const filename = path.basename(userRequest);
          const injected = injectChunks(content, userRequest, compilation);

          /* eslint-disable-next-line no-param-reassign */
          compilation.assets[filename] = {
            source: () => injected,
            size: () => injected.length,
          };
        }
      });
    });
    compiler.hooks.thisCompilation.tap(pluginName, (compilation) => {
      NormalModule.getCompilationHooks(compilation).loader.tap(pluginName, (loaderContext, module) => {
        if (isEntryModule(compilation, module)) {
          /* eslint-disable-next-line no-param-reassign */
          loaderContext[pluginName] = (content) => {
            entries[module.userRequest] = content;
          };
        } else {
          /* eslint-disable-next-line no-param-reassign */
          delete loaderContext[pluginName];
        }
      });
    });
  }
}

function traverseElements(element, callback) {
  callback(element);

  for (const child of element.childNodes) {
    traverseElements(child, callback);
  }
}

function loader(source) {
  this.cacheable();
  const callback = this.async();

  const root = parse(source, { script: true, style: true, pre: true });

  const options = loaderUtils.getOptions(this);

  const scripts = [];
  const links = [];

  traverseElements(root, (element) => {
    if (!element.tagName) {
      return;
    }

    if (element.tagName === 'SCRIPT') {
      scripts.push(element);
    } else if (element.tagName === 'LINK') {
      links.push(element);
    }
  });

  const requires = [];

  for (const script of scripts) {
    const src = script.attributes.src;
    if (src && !(src.startsWith('http') || src.startsWith('//'))) {
      requires.push(src);
      script.parentNode.removeChild(script);
    }
  }

  for (const link of links) {
    const href = link.attributes.href;
    const rel = link.attributes.rel;
    if (rel && link.attributes.rel === 'stylesheet' && href && !(href.startsWith('http') || href.startsWith('//'))) {
      requires.push(`!!style-loader!css-loader!less-loader!${href}`);
      link.parentNode.removeChild(link);
    }
  }

  if (options.minimize) {
    root.removeWhitespace();
  }

  let result = root.toString();
  if (root.firstChild.tagName === 'html') {
    result = `<!doctype html>${result}`;
  }
  if (this[pluginName]) {
    this[pluginName](result);
    result = '';
  }

  const requireClauses = requires.map((x) => `require('${x}');`).join('\n');
  const resultJs = `${requireClauses}\nmodule.exports = ${JSON.stringify(result)}`;
  callback(null, resultJs);
}

module.exports = loader;
module.exports.EntryExtractPlugin = EntryExtractPlugin;
