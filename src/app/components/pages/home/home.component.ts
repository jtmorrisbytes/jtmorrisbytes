import { Component, OnInit } from '@angular/core';
import {IPage } from '@app/lib/page/ipage';
import { inherits } from 'util';
@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit, IPage {
  path: string;
  title: string;
  subtitle: string;
  titlebarText: string;
  constructor() {
    this.path ='home'
    this.titlebarText = this.title = 'jtmorrisbytes.com';
    this.subtitle = 'An experiment by Jordan Morris';

  }

  ngOnInit() {
  }

}
