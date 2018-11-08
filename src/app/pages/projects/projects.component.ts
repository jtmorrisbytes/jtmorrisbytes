import { Component, OnInit } from '@angular/core';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-projects',
  templateUrl: './projects.component.html',
  styleUrls: ['./projects.component.scss']
})
export class ProjectsComponent implements OnInit, IPage {
  path: string = 'projects';
  title: string;
  titlebarText:string
  constructor() {
    this.title = 'My Projects';
    this.titlebarText = this.title;
   }

  ngOnInit() {
  }

}
