import { Component, OnInit, ViewChild, Input } from '@angular/core';
import { AppRoutingModule } from '../app-routing.module';
const appConfig = require("../data/app.config.json");
const appRoutes = require('../data/routes.json').routes;
console.log(appConfig);
console.log("routes:")
console.log(appRoutes);
let router:AppRoutingModule = new AppRoutingModule();


@Component({
  selector: 'app-nav-main',
  templateUrl: './nav-main.component.html',
  styleUrls: ['./nav-main.component.scss']
})
export class NavMainComponent implements OnInit {
  navTitle:string;
  appTitle:string;
  navSubtitle:string;
  routes: Array<Object>;
  constructor() {
    this.routes = appRoutes;
    
    this.navTitle = appConfig.title;
    this.navSubtitle = appConfig.subtitle;
   }
  public addNavLink(name, path){
    
  }
  ngOnInit() {
  }

}
