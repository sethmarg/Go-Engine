import Browser
import Html exposing (Html, div)

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
    { board : Board}

type Msg 
    = Play Position
    | Pass
    | Resign

type alias Position = { row : Int, col : Int}

init : Model
init = 
    { board = List.repeat 19 (List.repeat 19 Nothing)}

update : Msg -> Model -> Model
update msg model = 
    case msg of 
        Play pos ->
            model
        Pass ->
            model
        Resign ->
            model

view : Model -> Html Msg
view model = 
    div [] []

map2d : (a -> b) -> List (List a) -> List (List b)
map2d fn list = 
    List.map (List.map fn) list