import { NgModule } from '@angular/core';
import { Routes, RouterModule, RoutesRecognized } from '@angular/router';
import {HomeComponent} from './pages/home/home.component';
import {AboutComponent} from '@app/pages/about/about.component';
import * as appRoutes from './data/routes.json';
import {E404Component} from '@app/pages/e404/e404.component';
import { dashCaseToCamelCase } from '@angular/compiler/src/util';
const routes = [
  {path: '', component: HomeComponent, name: 'home', abc: '123'}

];
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
  constructor() {
    // set up the route with references to the routes.
  }
  public buildRoutingTable() {
    console.log('building routing table');
    let routeNum = 0;
    for (routeNum = 0; routeNum < PageComponents.length; routeNum++) {
      const currentRoute = appRoutes[routeNum];
      if (currentRoute.path.startsWith('/')) {
        currentRoute.path = currentRoute.path.substr(1, currentRoute.length);
      }
      const currentComponent = PageComponents[routeNum];
      if (currentRoute.component === currentComponent.name) {
        routes.push({path: currentRoute.path, component: currentComponent});
      }
      console.log(routes);

    }
    return routes;
  }

}
