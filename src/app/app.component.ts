import { Component, Input } from '@angular/core';
import {NavMainComponent} from './components/nav-main/nav-main.component';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {
  appTitle: string;
  subtitle: string;
  public constructor() {
    this.appTitle = 'JTMorris.me';
    this.subtitle = 'An experiment by Jordan Morris';
  }
  
}
