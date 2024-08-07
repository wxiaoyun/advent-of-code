# [ Day 20: Pulse Propagation ](https://adventofcode.com/2023/day/20)

--- Day 20: Pulse Propagation ---With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.The machines are far apart and wired together with longcables. The cables don't connect to the machines directly, but rather to communicationmodulesattached to the machines that perform various initialization tasks and also act as communication relays.Modules communicate usingpulses. Each pulse is either ahigh pulseor alow pulse. When a module sends a pulse, it sends that type of pulse to each module in its list ofdestination modules.There are several different types of modules:Flip-flopmodules (prefix%) are eitheronoroff; they are initiallyoff. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, itflips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.Conjunctionmodules (prefix&)rememberthe type of the most recent pulse received fromeachof their connected input modules; they initially default to remembering alow pulsefor each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembershigh pulsesfor all inputs, it sends alow pulse; otherwise, it sends ahigh pulse.There is a singlebroadcast module(namedbroadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, thebutton module. When you push the button, a singlelow pulseis sent directly to thebroadcastermodule.After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.Pulses are always processedin the order they are sent. So, if a pulse is sent to modulesa,b, andc, and then moduleaprocesses its pulse and sends more pulses, the pulses sent to modulesbandcwould have to be handled first.The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> aIn this module configuration, the broadcaster has three destination modules nameda,b, andc. Each of these modules is a flip-flop module (as indicated by the%prefix).aoutputs tobwhich outputs tocwhich outputs to another module namedinv.invis a conjunction module (as indicated by the&prefix) which, because it has only one input, acts like aninverter(it sends the opposite of the pulse type it receives); it outputs toa.By pushing the button once, the following pulses are sent:button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> aAfter this sequence, the flip-flop modules all end upoff, so pushing the button again repeats the same sequence.Here's a more interesting example:broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> outputThis module configuration includes thebroadcaster, two flip-flops (namedaandb), a single-input conjunction module (inv), a multi-input conjunction module (con), and an untyped module namedoutput(for testing purposes). The multi-input conjunction moduleconwatches the two flip-flop modules and, if they're both on, sends alow pulseto theoutputmodule.Here's what happens if you push the button once:button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> outputBoth flip-flops turn on and a low pulse is sent tooutput! However, now that both flip-flops are on andconremembers a high pulse from each of its two inputs, pushing the button a second time does something different:button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> outputFlip-flopaturns off! Now,conremembers a low pulse from modulea, and so it sends only a high pulse tooutput.Push the button a third time:button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> outputThis time, flip-flopaturns on, then flip-flopbturns off. However, beforebcan turn off, the pulse sent toconis handled first, so itbriefly remembers all high pulsesfor its inputs and sends a low pulse tooutput. After that, flip-flopbturns off, which causesconto update its state and send a high pulse tooutput.Finally, withaon andboff, push the button a fourth time:button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> outputThis completes the cycle:aturns off, causingconto remember only low pulses and restoring all modules to their original states.To get the cables warmed up, the Elves have pushed the button1000times. How many pulses got sent as a result (including the pulses sent by the button itself)?In the first example, the same thing happens every time the button is pushed:8low pulses and4high pulses are sent. So, after pushing the button1000times,8000low pulses and4000high pulses are sent. Multiplying these together gives32000000.In the second example, after pushing the button1000times,4250low pulses and2750high pulses are sent. Multiplying these together gives11687500.Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button1000times, waiting for all pulses to be fully handled after each push of the button.What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?