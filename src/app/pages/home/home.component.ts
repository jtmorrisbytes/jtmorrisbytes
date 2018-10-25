import { Component, OnInit } from '@angular/core';
import {IPage } from '../ipage'
import { inherits } from 'util';
import { Page } from '../page';
@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent extends Page implements OnInit,IPage {
  
  constructor() { 
    super();
    this.name="HomePage";
    this.navLinkText = "Home"
    this.path = "";
    
  }
  
  ngOnInit() {
  }

}
