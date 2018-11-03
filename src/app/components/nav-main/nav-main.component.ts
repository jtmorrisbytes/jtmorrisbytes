



import { Component, OnInit, Injectable, inject } from '@angular/core';
import {TestService} from '@app/services/test-service.service';
// testing relative import of directory


@Component({
  selector: 'app-nav-main',
  templateUrl: './nav-main.component.html',
  styleUrls: ['./nav-main.component.scss']
})




export class AppNavComponent implements OnInit {

  navTitle = 'placeholder text';
  appTitle = 'placeholder text';
  navSubtitle: string;
  routes: any;
  constructor(private Test: TestService) {

    this.appTitle = 'placeholder-text';
    this.navTitle = 'App name placeholder';
    // this.routes = routes;

    // this.navTitle = appConfig.title;
    // this.navSubtitle = appConfig.subtitle;
   }
  public addNavLink(name, path) {

  }
  ngOnInit() {
    this.navTitle = this.Test.getGreeting();
  }

}
