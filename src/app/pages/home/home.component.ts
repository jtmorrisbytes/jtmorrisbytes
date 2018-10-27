import { Component, OnInit } from '@angular/core';
import {IPage } from '@app/lib/page/ipage'
import { inherits } from 'util';
import { Page } from '../../lib/page/page';
@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent extends Page implements OnInit,IPage {
  
  constructor() { 
    super();
    this.title="Welcome To jtmorrisbytes";
    this.subtitle= "An experiment by Jordan Morris"
    this.path = "";
    this.parent = null;
    
  }
  
  ngOnInit() {
  }

}
