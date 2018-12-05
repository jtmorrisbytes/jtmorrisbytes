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
  authorFName = 'Jordan';
  authorMName = 'Taylor';
  authorLName = 'Morris';
  authorName:string;
  authorQualities:Array<string>;
  websiteName= 'jtmorrisbytes.com';
  constructor() {
    this.path ='home'
    this.titlebarText='';
    this.authorQualities= [
      'Web Developer.',
      'IT Guy.',
      'Really Cool Dude.'
    ];
    this.subtitle ="";

  }

  ngOnInit() {
  }

}
