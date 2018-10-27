import { IPage } from "./ipage";
import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";

export class Page implements IPage {
    /// name:string
    title:string;
    subtitle:string;
    path:string;
    href:string;
    parent: Component | CommonModule
    constructor(){

    }
    
}
