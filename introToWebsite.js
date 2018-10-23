var JTMorrisBytes = {}
var say = console.log

JTMorrisBytes.isReadyToSpeak = false;




JTMorrisBytes.name = "Jordan T Morris"
JTMorrisBytes.age = 24;
JTMorrisBytes.language = "English"
JTMorrisBytes.protocol = "Modern English grammar"
JTMorrisBytes.wantsToConnectToAudience = true
JTMorrisBytes.lovesToCode = true
JTMorrisBytes.knowsEverything = false
JTMorrisBytes.makesMistakes = true
JTMorrisBytes.learnsFromMistakes = "true..... Well... Mostly :)"
JTMorrisBytes.getsFrustratedSometimesWhenThingsDontWorkAsPlanned = true
JTMorrisBytes.comesBackAfterMonthsOfHeadScracthingAndBrokenKeyboards = true
JTMorrisBytes.hasLotsOfBadHumor = true;
JTMorrisBytes.isConnectedToAudience = false;

var audience = {}

audience.name = "audience"
audience.language = "English"
audience.protocol = "Modern English grammar"
audience.IsListening = true;

JTMorrisBytes.connectTo = function connectTo(reciever){
    if(reciever.IsListening){
        this.isConnectedToAudience = true;
        say(`${this.name} is connected to ${reciever.name}`)
    }
    else{
        say(`${reciever.name} is not listening right now, so I cant connect with them`)
        this.isConnectedToAudience = false
    }
    
}

DoIntroToWebsite();

function DoIntroToWebsite(){
    var weUnderstandEachOther = false;
    say(`preparing to connect with ${audience.name}.. please wait.....`)
    weUnderstandEachOther = PrepareToCommunicate(JTMorrisBytes, audience);
    if(weUnderstandEachOther){
        JTMorrisBytes.connectTo(audience)
        if(JTMorrisBytes.isConnectedToAudience){
            say("yay Im connected to my audience! lets do this thing")
            printIntroParagraph();
        }

    }
    else{
        say("sigh, it looks like nobody is home or we dont understand each other")
    }

    
}









function PrepareToCommunicate(sender, reciever) {
    var readyToCommunicate = true
    var notReadyToCommunicate = false
    if (sender != undefined && reciever != undefined){
       say("Whew! looks like we are all here. wait, do you speak my language? checking now.")
       if(sender.language === reciever.language){
           say("YAY! we speak the same language theres a good chance that we will probaly be able to talk. ")
           if(sender.protocol === reciever.protocol){
               say("Houston... We finally discovered that aliens do speak our language. we are ready to make first contact");
               return readyToCommunicate
           }
       }
       else {
           say("If you can understand this, please use translation software" )
           return notReadyToCommunicate;
       }
    }
    else{
        say("Uh oh! it looks like one or more of us are missing from the equation. I cant speak if nobody is here. sad face. :(")
        
        return notReadyToCommunicate 
    }
    
}

function printIntroParagraph(){
    say(`Hello, My name is ${JTMorrisBytes.name} I am ${JTMorrisBytes.age} years old`)
}