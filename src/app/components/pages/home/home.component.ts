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
  authorName = 'Jordan T Morris';
  authorQualities:Array<string>;
  websiteName= 'jtmorrisbytes.com';
  constructor() {
    this.path ='home'
    this.titlebarText='';
    this.title = this.authorName;
    this.authorQualities= [
      'Developer.',
      'IT Guy.',
      'Really Cool Dude.'
    ];
    this.subtitle ="";

  }

  ngOnInit() {
  }

}
