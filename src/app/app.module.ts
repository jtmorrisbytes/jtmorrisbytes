import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppComponent } from './app.component';
import { AppNavComponent } from './components/nav-main/nav-main.component';
import { AppPagesModule } from '@app/pages/app-pages.module';
import { FooterComponent } from './components/footer/footer.component';
import { CommonModule } from '@angular/common';


@NgModule({
  imports: [
    CommonModule
  ],

  declarations: [
    AppComponent,
    AppNavComponent,
    FooterComponent,

  ],
  imports: [
    BrowserModule,
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
