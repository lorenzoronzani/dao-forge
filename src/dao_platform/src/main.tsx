import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import './index.css';
import { createBrowserRouter, Navigate, RouterProvider } from 'react-router';
import { Dashboard } from './pages/Dashboard';
import { paths } from './constants/paths';
import { CreateDaoPage } from './pages/CreateDaoPage';
import DaoDetailsPage from './pages/DaoDetailsPage';
import CreateVotingPage from './pages/CreateVotingPage';
import { DataLoader } from './loaders/DataLoader';
import DaoGuard from './guards/DaoGuard';

const router = createBrowserRouter([
  {
    path: paths.HOME,
    element: <App />,
    children: [
      {
        index: true,
        element: (
          <DataLoader>
            <Dashboard />
          </DataLoader>
        )
      },
      {
        path: paths.DAOS_CREATE,
        element: <CreateDaoPage />
      },
      {
        path: paths.DAOS_DETAILS,
        element: (
          <DataLoader>
            <DaoGuard>
              <DaoDetailsPage />
            </DaoGuard>
          </DataLoader>
        )
      },
      {
        path: paths.VOTINGS_CREATE,
        element: (
          <DataLoader>
            <DaoGuard>
              <CreateVotingPage />
            </DaoGuard>
          </DataLoader>
        )
      },
      {
        path: "*",
        element: <Navigate to="/" replace />
      },
    ]
  }
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
