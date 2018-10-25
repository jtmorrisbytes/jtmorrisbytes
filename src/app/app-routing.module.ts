import { NgModule } from '@angular/core';
import { Routes, RouterModule, RoutesRecognized } from '@angular/router';
import {HomeComponent} from "./pages/home/home.component";
import {AboutComponent} from './pages/about/about.component';
import * as appRoutes from './data/routes.json'
import {E404Component} from './pages/e404/e404.component';
import { dashCaseToCamelCase } from '@angular/compiler/src/util';
 
let routes = [
  {path: '', component: HomeComponent}

]

const PageComponents = [
  HomeComponent,
  AboutComponent
];

console.log(HomeComponent.name);
@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})

export class AppRoutingModule {
  public routes = [];
  constructor(){
    // set up the route with references to the routes.
  }

  public buildRoutingTable() {
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

}

export let SharedRoutingModule = new AppRoutingModule();