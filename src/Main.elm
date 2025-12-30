module Main exposing (main)

import Browser
import Html exposing (Html, div, img)
import Html.Attributes exposing (src, style)
import Html.Events exposing (onClick)

main : Program () Model Msg
main = Browser.sandbox
        { init = init
        , view = view
        , update = update
        }

type Color
  = Black
  | White

type alias State = Maybe Color

type alias Board = List (List State)

type alias Model = 
    { board : Board 
    , curPlayer : Color }

type Msg 
    = Play Position
    | Pass
    | Resign

type alias Position = { row : Int, col : Int}

init : Model
init = 
    { board = List.repeat 19 (List.repeat 19 (Just White))
    , curPlayer = Black}

update : Msg -> Model -> Model
update msg model = 
            { model |
                board = indexedMap2d 
                    (\x -> \y -> \state -> 
                        if x == pos.row && y == pos.col then
                            Just model.curPlayer
                        else 
                            state)
                    model.board,
                curPlayer = oppositeColor model.curPlayer
                }
        Play pos ->
            { model |
                board = indexedMap2d 
                    (\x -> \y -> \state -> 
                        if x == pos.row && y == pos.col then
                            Just model.curPlayer
                        else 
                            state)
                    model.board,
                curPlayer = oppositeColor model.curPlayer
                }
        Pass ->
            model
        Resign ->
            model

view : Model -> Html Msg
view model = 
    --div [] [img [src "Go Board.png"] []]
    --img [src "Go Board.png"] (List.map (div []) (map2d stateToImage model.board))
    --div [] (List.map (div []) (map2d stateToImage model.board))
    --div [style "position" "relative"] (img [src "Go Board.png"] [] :: (List.map (div [style "position" "relative"]) (map2d stateToImage model.board)))
    div [] (img [src "Go Board.png"] [] :: (List.map (div [style "position" "relative"]) (indexedMap2d indexedStateToImage model.board)))

map2d : (a -> b) -> List (List a) -> List (List b)
map2d fn list = 
    List.map (List.map fn) list

indexedMap2d : (Int -> Int -> a -> b) -> List (List a) -> List (List b)
indexedMap2d fn list =
    List.indexedMap
        (\x -> \column -> 
            List.indexedMap
                (\y -> \elem ->
                    fn x y elem)
                column
        )
        list

colorToImage : Color -> Html Msg
colorToImage color =
    case color of 
        Black -> 
            img [src "black.png"] []
        White -> 
            img [src "white.png"] []

indexedStateToImage : Int -> Int -> State -> Html Msg
indexedStateToImage x y state =
    div 
        [ onClick (Play (Position x y)) 
        , style "position" "relative"] 
        [ case state of 
            Just color -> 
                colorToImage color
            Nothing -> 
                div [] []
        ]

stateToImage : State -> Html Msg
stateToImage state = 
  case state of 
    Just color -> 
        colorToImage color
    Nothing -> 
        div [] []
        
oppositeColor : Color -> Color
oppositeColor color =
    case color of
        White -> Black
        Black -> White