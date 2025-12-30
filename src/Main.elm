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
    { board = List.repeat 19 (List.repeat 19 (Nothing))
    , curPlayer = Black}

update : Msg -> Model -> Model
update msg model = 
    case msg of 
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
    div [ style "background-image" "url('Go Board.png')"
        , style "display" "grid"
        , style "grid-template-columns" "repeat(19, 23.21px)"
        , style "grid-template-rows" "repeat(19, 23.21px)"
        , style "width" "441px"
        , style "height" "441px"] 
        ((List.map (div []) (indexedMap2d indexedStateToImage model.board)))

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

colorToImagePath : Color -> String
colorToImagePath color =
    case color of 
        Black -> "black.png"
        White -> "white.png"

indexedStateToImage : Int -> Int -> State -> Html Msg
indexedStateToImage x y state =
        case state of 
            Just color -> 
                img [ src (colorToImagePath color)
                    , onClick (Play (Position x y))
                    , style "width" "100%"
                    , style "height" "100%"
                    , style "display" "block" ] 
                    []
            Nothing -> 
                div [ onClick (Play (Position x y)) 
                    , style "width" "100%"
                    , style "height" "100%" ] 
                    []
        
oppositeColor : Color -> Color
oppositeColor color =
    case color of
        White -> Black
        Black -> White