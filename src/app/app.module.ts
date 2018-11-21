import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppComponent } from '@app/app.component';
import { AppNavComponent } from './components/nav-main/nav-main.component';
import { FooterComponent } from './components/footer/footer.component';
import { CommonModule } from '@angular/common';
import { AppConfig } from "@app/app.config";
import { AboutComponent } from './components/pages/about/about.component';
import { HomeComponent } from './components/pages/home/home.component';
import { E500Component } from './components/errors/e500/e500.component';
import { E404Component } from './components/errors/e404/e404.component';
import { RouterModule } from '@angular/router';

const routes = [
  {path:"", component: HomeComponent },
  {path:"index", component: HomeComponent},
  {path: 'about', component: AboutComponent},
  {path: "*", redirectTo: "index", pathMatch:"full"}
]






@NgModule({
  declarations: [
    
    AppComponent,
    AppNavComponent,
    FooterComponent,
    AboutComponent,
    HomeComponent,
    E500Component,
    E404Component

  ],
  imports: [
    RouterModule.forRoot(routes),
    BrowserModule.withServerTransition({appId: AppConfig.appID}),
    CommonModule,
    //AppRoutingModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
  constructor() {
    
  }

 }
 