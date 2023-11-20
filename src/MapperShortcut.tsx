import { ButtonShortcut } from "./ButtonShortcut"
import "./styles/components.scss"

export const MapperShortcut = () => {

    return (
        <div className="mapper-shortcut">
            <ButtonShortcut />
            <p>TO</p>
            <ButtonShortcut />
        </div>
    )
}
