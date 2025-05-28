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

const router = createBrowserRouter([
  {
    path: paths.HOME,
    element: <App />,
    children: [
      {
        index: true,
        element: <Dashboard />
      },
      {
        path: paths.DAOS_CREATE,
        element: <CreateDaoPage />
      },
      {
        path: paths.DAOS_DETAILS,
        element: <DaoDetailsPage />
      },
      {
        path: paths.VOTINGS_CREATE,
        element: <CreateVotingPage />
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
