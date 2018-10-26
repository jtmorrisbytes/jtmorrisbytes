



import { Component, OnInit, ViewChild, Input } from '@angular/core';
// testing relative import of directory
var testJsonAsset = "assets/test.json"
console.log(`importing test asset ${testJsonAsset} using require`)

var appConfig = require("@app/assets/app.config.json")
import { RouteReuseStrategy } from '@angular/router';
// import * as appRoutes from "../data/routes.json";
//console.log(appConfig);
console.log(appConfig.title)


@Component({
  selector: 'app-nav-main',
  templateUrl: './nav-main.component.html',
  styleUrls: ['./nav-main.component.scss']
})
export class NavMainComponent implements OnInit {
  navTitle:string = "placeholder text";
  appTitle:string = "placeholder text";
  navSubtitle:string;
  routes: any;
  constructor() {
    this.appTitle = "placeholder-text"
    this.navTitle = "App name placeholder"
    //this.routes = routes;
    
    // this.navTitle = appConfig.title;
    // this.navSubtitle = appConfig.subtitle;
   }
  public addNavLink(name, path){
    
  }
  ngOnInit() {
  }

}
