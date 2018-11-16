import { Component, OnInit } from '@angular/core';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-e500',
  templateUrl: './e500.component.html',
  styleUrls: ['./e500.component.scss']
})
export class E500Component implements OnInit, IPage {
  path: string = '500';
  title: string;
  titlebarText: string;
  constructor() { }

  ngOnInit() {
  }

}
