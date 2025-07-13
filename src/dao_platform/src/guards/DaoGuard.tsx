import React, { ReactNode, useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router';
import { useDao } from '@/providers/DaoProvider';
import { Principal } from '@dfinity/principal';
import { paths } from '@/constants/paths';

interface DaoGuardProps {
    children: ReactNode;
}

export const DaoGuard: React.FC<DaoGuardProps> = ({ children }) => {
    const { id } = useParams<{ id: string }>();
    const { getDao } = useDao();
    const navigate = useNavigate();
    const [isValid, setIsValid] = useState(false);

    useEffect(() => {
        try {
            const principal = Principal.fromText(id!);
            getDao(principal);

            setIsValid(true);
        } catch (error) {
            navigate(paths.HOME);
        }
    }, [id, getDao, navigate]);

    return isValid ? <>{children}</> : null;
};

export default DaoGuard;
