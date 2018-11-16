///< reference types="node" >

import { Injectable } from "@angular/core";
import { readFile, readFileSync } from 'fs';
import * as walk from "walk";
//directory provider service is intended to walk a directory and look for modules and components and ( at some point ) provide a reference to the data of said modules or components.
@Injectable({
    providedIn: 'root'
})


export class DirectoryProvider {
    rootDirectory: string = null;
    nullRootDirectoryError: ReferenceError;
    constructor() {
        
    }
    setRootDirectory(newRootDirectory: string) {
        this.rootDirectory = newRootDirectory;
    }
    getRootDirectory(): {error?: ReferenceError, rootDirectory: string} {
        let result = {
            error: null,
            rootDirectory: null
        };
        if (this.rootDirectory === null || this.rootDirectory === undefined) {
            this.nullRootDirectoryError =
            ReferenceError(`the rootDirectory was null while attempting to call getRootDirectory()
            on class ${DirectoryProvider.name} `);
            result.error = this.nullRootDirectoryError;
            
        }
        else result.rootDirectory = this.rootDirectory;
        return result;
    }
    walkDirectoryTree() {
    }
}