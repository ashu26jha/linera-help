function Navbar() {
    return (
        <div>
            <div className="nav-wrapper">
                <div className="logo">
                    <img src="Logo.png" />
                </div>
                <div className="nav-items">
                    <a href="/mint"><div className="nav-item">Mint</div></a>
                    <a href="/list"><div className="nav-item">List</div></a>
                    <a href="/buy"><div className="nav-item">Buy</div></a>
                    <a href="/chat"><div className="nav-item">Chat</div></a>
                </div>
            </div>
        </div>
    )
}
export default Navbar;
