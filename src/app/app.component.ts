import { Component, Input, Inject, PLATFORM_ID } from '@angular/core';
import {AppNavComponent} from './components/nav-main/nav-main.component';
import { isPlatformBrowser } from '@angular/common';
declare var flowtype: Function;
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {
  title: string;
  subtitle: string;
  public constructor(@Inject(PLATFORM_ID) private platformId) {
    this.title = 'jtmorrisbytes.com';
    this.subtitle = 'An experiment by Jordan Morris';
  }
}
