import { Component, OnInit } from '@angular/core';
import {IPage } from '@app/lib/page/ipage';
import { inherits } from 'util';
import { Page } from '../../lib/page/page';
@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit, IPage {
  static path = 'home';
  someStaticStringTest = "staticStringSuccess";
  title: string;
  subtitle: string;
  titlebarMsg: string;
  constructor() {
    this.titlebarMsg = this.title = 'Welcome To jtmorrisbytes';
    this.subtitle = 'An experiment by Jordan Morris';

  }

  ngOnInit() {
  }

}
