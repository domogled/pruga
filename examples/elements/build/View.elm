module View exposing(view)

import Html exposing (Html)
import Html exposing (li, span, text, ul)
import Html.Attributes exposing (class, id)

view : Model -> Html Msg
view model =
            ul
                [ id "ulID"
                , class "uluju class"
                ]
                [ li
                    [ id "L1"
                    , class "liju class"
                    ]
                    [ text "pre text"
                    , span [id "S1"]
                        [ text "text 1"
                        ]
                    , text "post text"
                    ]
                , li
                    [ id "L2"
                    , class "liju class"
                    ]
                    [ span [id "S2"]
                        [ text "text 2"
                        ]
                    ]
                , li
                    [ id "L3"
                    , class "liju class"
                    ]
                    [ span [id "S3"]
                        [ text "text 3"
                        ]
                    ]
                , li
                    [ id "L4"
                    , class "liju class"
                    ]
                    [ span [id "S4"]
                        [ text "text 4"
                        ]
                    ]
                ]