@startuml
title Tweak Charge Schedule

[*] --> PickBus : Given charge schedule

' Pick generator
PickBus --> PickGenerator
PickGenerator : - New visit
PickGenerator : - Slide visit
PickGenerator : - New charger
PickGenerator : - Remove
PickGenerator : - New window

' Execute selected generator
PickGenerator --> ExecuteGenerator

ExecuteGenerator --> [*]

@enduml
