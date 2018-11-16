import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppComponent } from '@app/app.component';
import { AppNavComponent } from './components/nav-main/nav-main.component';
import { FooterComponent } from './components/footer/footer.component';
import { CommonModule } from '@angular/common';
import { AppConfig } from "@app/app.config";
@NgModule({
  declarations: [
    AppComponent,
    AppNavComponent,
    FooterComponent,

  ],
  imports: [
    BrowserModule.withServerTransition({appId: AppConfig.appID}),
    CommonModule,
    //AppRoutingModule,
    AppPagesModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
  constructor() {
    
  }

 }
 