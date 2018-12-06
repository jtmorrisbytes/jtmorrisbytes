import { Component, OnInit } from '@angular/core';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-about',
  templateUrl: './about.component.html',
  styleUrls: ['./about.component.scss']
})
export class AboutComponent implements OnInit, IPage {
  path          = 'about';
  title         = 'About this website';
  titlebarText  = 'About jtmorrisbytes.com';
  constructor() { }

  ngOnInit() {
  }

}
