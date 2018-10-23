import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { NavMainComponent } from './nav-main/nav-main.component';
import { HomeComponent } from './home/home.component';
import { PageNotFoundComponent } from './page-not-found/page-not-found.component';
import { AboutComponent } from './about/about.component';
import { MyProjectsComponent } from './my-projects/my-projects.component';


@NgModule({
  declarations: [
    AppComponent,
    NavMainComponent,
    HomeComponent,
    PageNotFoundComponent,
    AboutComponent,
    MyProjectsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
 }
