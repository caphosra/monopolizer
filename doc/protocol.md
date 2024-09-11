# Protocol for monopolizer

|Path|Method|Arguments|Response|Description|
|:--|:--:|:--|:--|:--|
|`/init`|`GET`|`num: number`|`IGameInfo`|Get an initialized game|
|`/step`|`POST`|`{game: IGameInfo, num: number}`|`IGameInfo`|Simulate `n` turns|
|`/places`|`POST`|`IGameInfo`|`{places: IPlaceProp[]}`|Get properties of the places|
|`/tap`|`POST`|`IGameInfo`|`{taps: number[]}`|Get TAP|
|`/money`|`POST`|`IGameInfo`|`{money: number[], available: number[], total: number[]}`|Analyze a board in terms of money|
|`/survival`|`POST`|`{game: IGameInfo, num: number, depth: number}`|`{survival_rates: number[]}`|Simulate the game to calculate the survival rates.|
