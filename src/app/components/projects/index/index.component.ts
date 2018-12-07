import { Component, OnInit } from '@angular/core';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-projects',
  templateUrl: './index.component.html',
  styleUrls: ['./index.component.scss']
})
export class ProjectsIndexComponent implements OnInit, IPage {
  path = 'projects';
  title: string;
  titlebarText: string;
  constructor() {
    this.title = 'My Projects';
    this.titlebarText = this.title;
   }

  ngOnInit() {
  }

}
