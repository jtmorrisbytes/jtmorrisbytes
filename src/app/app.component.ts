import { Component, Input } from '@angular/core';
import {AppNavComponent} from './components/nav-main/nav-main.component';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {
  title: string;
  subtitle: string;
  public constructor() {
    this.title = 'jtmorrisbytes.com';
    this.subtitle = 'An experiment by Jordan Morris';
  }
  
}
