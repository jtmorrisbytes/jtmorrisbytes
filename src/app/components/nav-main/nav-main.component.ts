import { Component, OnInit, Injectable, inject, Inject, PLATFORM_ID } from '@angular/core';
import {TestService} from '@app/services/test-service.service';
import * as $ from "jquery";
import { NavigationProviderService } from '@app/services/navigation/navigation-provider.service'
import {debounce } from "underscore";
import { isPlatformBrowser } from '@angular/common';
// testing relative import of directory


@Component({
  selector: 'app-nav-main',
  templateUrl: './nav-main.component.html',
  styleUrls: ['./nav-main.component.scss']
})




export class AppNavComponent implements OnInit {
  navLinksSelector:string;
  /* ENSURE THAT YOU UPDATE largeDevicesWidth according to css breakpoints
     or navbar menu toggling will break;
  */
  largeDevicesWidth = 576;
  navTitle = 'placeholder text';
  appTitle = 'placeholder text';
  navSubtitle: string;
  routes: any;
  isBrowser;
   
  constructor(@Inject(PLATFORM_ID) private platformId,private Test: TestService, private navigationProvider: NavigationProviderService) {
    this.navLinksSelector = "app-nav-links";
    this.appTitle = 'placeholder-text';
    this.navTitle = 'App name placeholder';
    this.isBrowser = isPlatformBrowser(platformId);
    if (this.isBrowser) {
      window.addEventListener("resize",
      debounce((event) => this.removeDisplayAttributeWhenNoLongerMobileWidth(event), 2));
     }
    }
   
  calcDocumentWidth() {
    return Math.max(
      document.documentElement["clientWidth"],
      document.body["scrollWidth"],
      document.documentElement["scrollWidth"],
      document.body["offsetWidth"],
      document.documentElement["offsetWidth"]
    );
  }
  isSmallDevice() {
    return this.calcDocumentWidth() < this.largeDevicesWidth;
  }
  isLargeDevice() {
    return this.calcDocumentWidth() >= this.largeDevicesWidth;
  }
  removeDisplayAttributeWhenNoLongerMobileWidth(event) {
    if(this.isLargeDevice()) {
      document.getElementById("app-nav-links").style.display = "";
    }
    
  }
  onDropdownButtonClicked() {
    // calculation credits to jquery
    
    const turnOnGrid = () => {
          // $(this.navLinksSelector).css("display","grid");
          
    }
    //inital state: display none.
    // on the start of the animation, set the display property to grid
    if (this.isSmallDevice()) {
      if (document.getElementById(this.navLinksSelector).style.display === "none"||
        document.getElementById(this.navLinksSelector).style.display === "") 
      {
          document.getElementById(this.navLinksSelector).style.display = "grid";
      }
      else {
        document.getElementById(this.navLinksSelector).style.display = "none";
      }
    }


    
   

  }
  ngOnInit() {
    this.navTitle = this.Test.getGreeting();
  }

}
