# Protocol for monopolizer

|Path|Method|Arguments|Response|Description|
|:--|:--:|:--|:--|:--|
|`/init`|`GET`|`num: number`|`IGameInfo`|Get an initialized game|
|`/step`|`POST`|`{game: IGameInfo, num: number}`|`IGameInfo`|Simulate `n` turns|
|`/places`|`POST`|`IGameInfo`|`{places: IPlaceProp[]}`|Get properties of the places|
