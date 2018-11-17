import { Component, OnInit } from '@angular/core';
import { IErrorPage } from '@app/lib/page/IErrorPage';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-e404',
  templateUrl: './e404.component.html',
  styleUrls: ['./e404.component.scss']
})
export class E404Component implements OnInit, IPage, IErrorPage {
  title: string;
  titlebarText: string;
  errorCode = 404;
  message: string;
  path: string;
  constructor() {
    this.titlebarText = `Error ${this.errorCode}!`;
    this.title = "These are not the code bytes you are looking for !";
    this.message  = " the resource you are looking for was not found."
  }

  ngOnInit() {
  }

}
