import { app, BrowserWindow } from 'electron';
import * as path from 'path';

let mainWindow = null;

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('ready', () => {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  // eslint-disable-next-line no-console
  mainWindow.loadURL(`file://${__dirname}/index.html`).catch((e) => { console.error(e); });
  mainWindow.on('closed', () => {
    mainWindow = null;
  });
});
