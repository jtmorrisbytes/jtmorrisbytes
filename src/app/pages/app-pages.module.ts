import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {HomeComponent} from './home/home.component'
import {AboutComponent} from './about/about.component'
import {ProjectsModule} from "./projects/projects.module"
import { E404Component } from './e404/e404.component'
import { E500Component } from './e500/e500.component'

@NgModule({
  imports: [
    CommonModule,
    ProjectsModule
  ],
  declarations: [
    HomeComponent,
    AboutComponent,

    E404Component,
    
    E500Component,
  ]
})
export class AppPagesModule { }
