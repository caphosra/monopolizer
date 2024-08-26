# Standard JSON structure for Monopoly games

## `IPlayerInfo`

```json
{
    "player_id": 0, // The id of the player
    "money": 1500, // The amount of money the player has
    "is_bankrupted": false, // A flag indicating whether the player is bankrupted
    "jail_turn": 1, // [Optional] The number of turns the player has been in jail
    "position": 40, // The position of the player
}
```

## `IPlaceInfo`

```json
{
    "place_id": 0, // The id of the place
    "owner": 0, // [Optional] The id of the player who owns the place
    "is_mortgaged": false, // A flag indicating whether the place is mortgaged
    "houses": 0, // [Optional] The number of houses on the place
}
```

## `IGameInfo`

```json
{
    "turn": 0, // The id of the current player
    "players": [
        {
            "player_id": 0, // The id of the player
            "money": 1500, // The amount of money the player has
            "is_bankrupted": false, // A flag indicating whether the player is bankrupted
            "jail_turn": 1, // [Optional] The number of turns the player has been in jail
            "position": 40, // The position of the player
        }
    ],
    "places": [
        {
            "place_id": 0, // The id of the place
            "owner": 0, // [Optional] The id of the player who owns the place
            "is_mortgaged": false, // A flag indicating whether the place is mortgaged
            "houses": 0, // [Optional] The number of houses on the place
        }
    ]
}
```

## `IPlaceProp`

```json
{
    "place_id": 0, // The id of the place
    "name": "Boardwalk", // The name of the place
    "color": "Blue", // The color of the place
    "price": 400, // [Optional] The price of the place
    "house_price": 200, // [Optional] The price of a house on the place
    "rent": 50, // [Optional] The rent of the place
}
```
