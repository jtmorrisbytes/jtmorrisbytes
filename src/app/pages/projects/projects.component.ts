import { Component, OnInit } from '@angular/core';
import { Page } from '@app/lib/page/page';
import { IPage } from '@app/lib/page/ipage';

@Component({
  selector: 'app-projects',
  templateUrl: './projects.component.html',
  styleUrls: ['./projects.component.scss']
})
export class ProjectsComponent implements OnInit, IPage {
  static path = 'projects';
  title: string;
  constructor() {
    this.title = 'My Projects';
   }

  ngOnInit() {
  }

}
