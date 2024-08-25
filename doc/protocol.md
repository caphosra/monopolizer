# Protocol for monopolizer

|Path|Method|Arguments|Response|Description|
|:--|:--:|:--|:--|:--|
|`/init`|`GET`|`num: number`|`IGameInfo`|Get an initialized game|
|`/step`|`GET`|`{game: IGameInfo, num: number}`|`IGameInfo`|Simulate `n` turns|
