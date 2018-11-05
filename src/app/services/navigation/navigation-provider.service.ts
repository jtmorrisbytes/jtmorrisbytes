import { Injectable } from '@angular/core';
import { HomeComponent } from '@app/pages/home/home.component';
import { AboutComponent } from '@app/pages/about/about.component';
import { ProjectsComponent } from '@app/pages/projects/projects.component';
import { forEach } from '@angular/router/src/utils/collection';
import { IPage } from '@app/lib/page/ipage';
@Injectable({
  providedIn: 'root'
})
export class NavigationProviderService {
  pages:any = [
    HomeComponent,
    AboutComponent,
    ProjectsComponent
  ];
  routes: [{}];
  constructor() {
    this.routes =
    [
      {path: HomeComponent.path}
    ];
  }
  generateNavLinks(){
    for (let component in this.pages) {
      this.routes.push({path: component.path});

    }
  }
}
