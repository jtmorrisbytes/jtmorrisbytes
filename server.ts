// These are important and needed before anything else
import 'zone.js/dist/zone-node';
import 'reflect-metadata';

import { enableProdMode } from '@angular/core';

import * as express from 'express';
import { join } from 'path';

// Faster server renders w/ Prod mode (dev mode never needed)
enableProdMode();

// Express server
const app = express();
const IP_ADDR = process.env.IP_ADDR || "0.0.0.0";
const PORT = process.env.PORT || 3000;
const DIST_FOLDER = join(process.cwd(), 'dist');

// * NOTE :: leave this as require() since this file is built Dynamically from webpack
const { AppServerModuleNgFactory, LAZY_MODULE_MAP } = require('./dist/server/main');

// Express Engine
import { ngExpressEngine } from '@nguniversal/express-engine';
// Import module map for lazy loading
import { provideModuleMap } from '@nguniversal/module-map-ngfactory-loader';

app.engine('html', ngExpressEngine({
  bootstrap: AppServerModuleNgFactory,
  providers: [
    provideModuleMap(LAZY_MODULE_MAP)
  ]
}));

app.set('view engine', 'html');
app.set('views', join(DIST_FOLDER, 'browser'));
app.set('ip_address', IP_ADDR);
// TODO: implement data requests securely
app.get('/api/*', (req, res) => {
  res.status(404).send('data requests are not supported');
});
// jtmorrisbytes.com ssl challenge
app.get('/.well-known/acme-challenge/0mpb_J0EKcWrg3vvOgSHHGfu0mjM91AdqVbWZoK-B5s', (req, res) =>{
  res.status(200).send('0mpb_J0EKcWrg3vvOgSHHGfu0mjM91AdqVbWZoK-B5s.JRlKHWTOTxUBCaKP2GALYthdyEUQqSU85hHW9s4kmK0')
});
// www.jtmorrisbytes.com ssl challenge
app.get('/.well-known/acme-challenge/BJg-6DjvAq0ow6DmEn_VgIBjpcUuOhKDogSlkFHdScw', (req, res) => {
  res.status(200).send("BJg-6DjvAq0ow6DmEn_VgIBjpcUuOhKDogSlkFHdScw.JRlKHWTOTxUBCaKP2GALYthdyEUQqSU85hHW9s4kmK0")
});
// Server static files from /browser
app.get('*.*', express.static(join(DIST_FOLDER, 'browser')));

// All regular routes use the Universal engine
app.get('*', (req, res) => {
  res.render('index', { req });
});

// Start up the Node server
app.listen(PORT, IP_ADDR, () => {
  console.log(`Node server listening on http://${IP_ADDR}:${PORT}`);
});
