import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';

// IPage contains the common data structure all my webpages should have

export interface IPage {
    /* title: the title of the webpage, usually a few buzzwords
    // to catch the attention of the reader. should lead into the subtitle, if
     one is provided
    */
    
    title: string;

    /* subtitle: A short extension of the title meant to add to the meaning of the title
       usually displayed with the title.
    */
    subtitle?: string;
    /*
        path: the portion of the url excluding the leading forward slash
        that defines where to find the webpage
    */
    path:string;
    /*
        parentModule: a reference to the parent module if it exists;
    */

     titlebarText:string;
}
