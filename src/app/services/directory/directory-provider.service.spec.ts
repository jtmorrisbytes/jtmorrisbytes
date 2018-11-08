import { TestBed, async } from '@angular/core/testing';

import { DirectoryProvider } from './directory-provider.service';
it("should set the root directory when calling setRootDirectory()", () => {
        const testdirectoryString = "/home";
        provider.setRootDirectory(testdirectoryString);
        const result = provider.getRootDirectory();
        expect(result.rootDirectory).toBeTruthy();
        expect(result.error).toBeNull();

     })
describe('DirectoryProvider', () => {
    let provider: DirectoryProvider;
  beforeEach(async(() => {
    TestBed.configureTestingModule({});
    provider =  new DirectoryProvider();
    console.log(provider);
  }));

  it('should be created', () => {
    
    expect(provider).toBeTruthy();
    
  });
  it('should have a rootDirectory property', () => {
    expect(provider.rootDirectory).toBeDefined();
  });
  it(" should return an object with an error and a null root directory when calling getRootDirectory() " +
     "without setting a rootDirectoryProperty",
     () => {
      const result = provider.getRootDirectory();
      expect(result).toBeDefined("Expected getRootDirectory() to return an object when called");

      expect(result.error).toBeDefined(
        "Expected getRootDirectory() to return an error object when the programmer" +
        "failed to explicitly define a root directory"
      );
      expect(result.error.name).toBe(ReferenceError.name,
         `getRootDirectory() returned object with name ${result.error.name} ` +
         `but was expecting a ReferenceError object with the name ${ReferenceError.name}`);
      expect(result.rootDirectory).toBeDefined("Expected getRootDirectory() to return a property named rootDirectory");
      expect(result.rootDirectory).toBeNull(
        'Expected getRootDirectory to return the object rootDirectory with a null value when' +
        'called before calling setRootDirectory()'
        );
     });
     it("should have the function setRootDirectory()", () => {
       expect(provider.setRootDirectory).toBeTruthy();
     })
     it("should set the root directory when calling setRootDirectory()", () => {
        const testdirectoryString = "/home";
        provider.setRootDirectory(testdirectoryString);
        const result = provider.getRootDirectory();
        expect(result.rootDirectory).toBeTruthy();
        expect(result.error).toBeNull();

     })
});
