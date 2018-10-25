import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { NavMainComponent } from './components/nav-main/nav-main.component';
import { HomeComponent } from './pages/home/home.component';
import { AboutComponent } from './pages/about/about.component';
import { ProjectsModule } from './pages/projects/projects.module';
import { E500Component } from './pages/e500/e500.component';
import { E404Component } from './pages/e404/e404.component';
import { AppPagesModule } from "./pages/app-pages.module"


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
 }
