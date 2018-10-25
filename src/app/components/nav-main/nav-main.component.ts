


var testdata = require("./data.json")
import { Component, OnInit, ViewChild, Input } from '@angular/core';
//import * as routes from '../../../assets/routes.json';
import * as _ from "lodash"
//testing a theory here


console.log(testdata);

import { RouteReuseStrategy } from '@angular/router';
// import * as appRoutes from "../data/routes.json";
//console.log(appConfig);
console.log("routes:")


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
