@startuml

title Charge Schedule Generation

[*] --> PickBus : Given route schedule
PickBus : Pick a random ID

PickBus --> PickRoute
PickRoute : Pick a random route associated with selected ID

PickRoute --> ListValidRegions
ListValidRegions : Run New Visit generator

ListValidRegions --> PickValidRegion
PickValidRegion : Randomly select one of the valid regions

PickValidRegion --> PickBus : While there are still \nbuses to assign time slots
PickValidRegion --> [*] : After each bus has been assigned a slot \nor no valid time slot for a bus has been found\nreturn either schedule or error

@enduml
