(() => {
  // eslint-disable-next-line @typescript-eslint/require-await
  async function getBaseUri(): Promise<string> {
    if (process.env.IS_LOCALHOST) return 'http://localhost:8000/compressed';
    return 'https://ffxiv-data.dlunch.net/compressed';
  }

  async function start(): Promise<void> {
    const pkg = await import('./pkg');

    pkg.start(await getBaseUri());
  }

  // eslint-disable-next-line no-console
  start().catch(console.error);
})();
