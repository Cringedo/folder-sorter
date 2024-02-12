// Modal.js
import React from 'react';
import './App.css'; // You can create a separate CSS file for styling

interface ModalProps  {
    show: boolean,
    handleClose: () => void;
    children: React.ReactNode; 
}

const Modal: React.FC<ModalProps> = ({ show, handleClose, children }) => {
    const showHideClassName = show ? 'modal display-block' : 'modal display-none';

    return (
        <div className={showHideClassName}>
            <section className="modal-main">
                <p className='side-components' style={{paddingRight: '40px'}}>{children}</p> 
                <button className='button-submit' style={{width: "5%", height: "10%", padding:"10px"}} onClick={handleClose}>X</button>
            </section>
        </div>
    );
};

export default Modal;
