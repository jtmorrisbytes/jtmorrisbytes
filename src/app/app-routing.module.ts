import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import {HomeComponent} from "./home/home.component";
import {AboutComponent} from './about/about.component';
import {MyProjectsComponent} from "../app/my-projects/my-projects.component";

import {pageNotFoundComponent} from './page-not-found/page-not-found.component';
var appRoutes = require("./data/routes.json").routes


const PageComponents = [
  HomeComponent,
  MyProjectsComponent,
  AboutComponent
];


function buildRoutingTable() {
  let routes = [];
  console.log("building routing table")
  let routeNum = 0;
  for(routeNum = 0; routeNum < PageComponents.length; routeNum++){
    let currentRoute = appRoutes[routeNum];
    if(currentRoute.path.startsWith("/")){
      currentRoute.path = currentRoute.path.substr(1,currentRoute.length);
    }
    let currentComponent = PageComponents[routeNum];
    if(currentRoute.component ===currentComponent.name){
      routes.push({path:currentRoute.path, component:currentComponent});
    }
    console.log(routes);
    
  }
  return routes;
}
console.log(HomeComponent.name);
@NgModule({
  imports: [RouterModule.forRoot(buildRoutingTable()],
  exports: [RouterModule]
})


export class AppRoutingModule {
  public routes = [];
  constructor(){
    // set up the route with references to the routes.
  }
  public _routes = [
    {path: '', component: HomeComponent}
  ];

  }
}
