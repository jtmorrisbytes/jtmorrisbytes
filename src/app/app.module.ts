import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { NavMainComponent } from './components/nav-main/nav-main.component';
import { HomeComponent } from '@app/pages/home/home.component';
import { AboutComponent } from '@app/pages/about/about.component';
import { ProjectsModule } from '@app/pages/projects/projects.module';
import { E500Component } from '@app/pages/e500/e500.component';
import { E404Component } from '@app/pages/e404/e404.component';
import { AppPagesModule } from "@app/pages/app-pages.module"


@NgModule({
  declarations: [
    AppComponent,
    NavMainComponent,
    
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    AppPagesModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
  constructor(){
    new AppPagesModule()

  }

 }
