PGDMP  :                     }            mtnnigeria_db    17.2    17.2     �           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                           false            �           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                           false            �           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                           false            �           1262    16404    mtnnigeria_db    DATABASE     �   CREATE DATABASE mtnnigeria_db WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_United States.1252';
    DROP DATABASE mtnnigeria_db;
                     postgres    false            �            1259    16430 
   department    TABLE     �   CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);
    DROP TABLE public.department;
       public         heap r       postgres    false            �            1259    16405 	   employees    TABLE     �   CREATE TABLE public.employees (
    eid integer NOT NULL,
    ename text NOT NULL,
    dno integer NOT NULL,
    esal real,
    age integer NOT NULL,
    phone text NOT NULL
);
    DROP TABLE public.employees;
       public         heap r       postgres    false            �            1259    16437    project    TABLE     �   CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    project_managerid integer NOT NULL
);
    DROP TABLE public.project;
       public         heap r       postgres    false            �          0    16430 
   department 
   TABLE DATA           P   COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
    public               postgres    false    218   �       �          0    16405 	   employees 
   TABLE DATA           F   COPY public.employees (eid, ename, dno, esal, age, phone) FROM stdin;
    public               postgres    false    217   `       �          0    16437    project 
   TABLE DATA           K   COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
    public               postgres    false    219   }       ^           2606    16436    department department_pkey 
   CONSTRAINT     d   ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);
 D   ALTER TABLE ONLY public.department DROP CONSTRAINT department_pkey;
       public                 postgres    false    218            \           2606    16411    employees employees_pkey 
   CONSTRAINT     W   ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);
 B   ALTER TABLE ONLY public.employees DROP CONSTRAINT employees_pkey;
       public                 postgres    false    217            `           2606    16443    project project_pkey 
   CONSTRAINT     a   ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);
 >   ALTER TABLE ONLY public.project DROP CONSTRAINT project_pkey;
       public                 postgres    false    219            �   �   x�M���0F�O�������@�88�\�M)hk���D�zrr��(T��?������!υ�
��q		�{pGPj��#9�����%r\yf�l�[��q,Q�"-��Eh��{�߼]g���;�}K���E!��G�/�      �     x�-�Mn�0���������%RWӎ��K��	U���u�f=`X�6���	�8(�����������PE�Y�9� �L�-�����1'�� � F�����#�jZ��p�� Ts�`:v�H5�l���eܧS������IOŝ�j�9݊.�+3����ac�0��u\���<�!K̎��̜>v����tָ��e!f[ț�"|��x�����I%�D�yK�!=���ܾ�CW��'�	ɇ��^���j�\�      �   J   x�34�t�T���+�(�440�22�t�44�	Y�ss:s��p��p�p�"�Yp��r�"�d����� I�     